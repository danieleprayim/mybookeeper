import { useEffect, useState } from "react";
import { journalApi } from "../../apis/journal";

type Line = {
    account_id: string;
    debit: number;
    credit: number;
};

type JournalType = "manual" | "sale" | "purchase" | "wages";

export default function JournalForm({ onSuccess, onCancel }: any) {
    const [journalType, setJournalType] = useState<JournalType>("manual");

    const [date, setDate] = useState("");
    const [description, setDescription] = useState("");
    const [lines, setLines] = useState<Line[]>([
        { account_id: "", debit: 0, credit: 0 },
    ]);
    const [error, setError] = useState("");

    // Templates
    const templates: Record<Exclude<JournalType, "manual">, Line[]> = {
        sale: [
            { account_id: "Cash", debit: 0, credit: 0 },
            { account_id: "Revenue", debit: 0, credit: 0 },
        ],
        purchase: [
            { account_id: "Inventory", debit: 0, credit: 0 },
            { account_id: "Cash", debit: 0, credit: 0 },
        ],
        wages: [
            { account_id: "Wages Expense", debit: 0, credit: 0 },
            { account_id: "Cash", debit: 0, credit: 0 },
        ],
    };

    // Apply template when type changes
    useEffect(() => {
        if (journalType === "manual") {
            setLines([{ account_id: "", debit: 0, credit: 0 }]);
        } else {
            setLines(templates[journalType]);
        }
    }, [journalType]);

    const addLine = () => {
        if (journalType !== "manual") return; // 🔒 lock for template mode
        setLines([...lines, { account_id: "", debit: 0, credit: 0 }]);
    };

    const removeLine = (index: number) => {
        if (journalType !== "manual") return; // 🔒 lock for template mode
        setLines(lines.filter((_, i) => i !== index));
    };

    const resetForm = () => {
        setDate("");
        setDescription("");
        setLines([{ account_id: "", debit: 0, credit: 0 }]);
        setError("");
        setJournalType("manual");
    };

    const handleCancel = () => {
        const hasData =
            date ||
            description ||
            lines.some((l) => l.account_id || l.debit || l.credit);

        if (hasData && !confirm("Perubahan data akan hilang. Lanjutkan?")) return;

        resetForm();
        onCancel?.();
    };

    const updateLine = <K extends keyof Line>(
        index: number,
        field: K,
        value: Line[K]
    ) => {
        const updated = [...lines];

        if (field === "account_id") {
            updated[index][field] = value;
        } else {
            updated[index][field] = Number(value) as Line[K];

            // enforce one side only (accounting rule)
            if (field === "debit") updated[index].credit = 0;
            if (field === "credit") updated[index].debit = 0;
        }

        setLines(updated);
    };

    const totalDebit = lines.reduce((sum, l) => sum + l.debit, 0);
    const totalCredit = lines.reduce((sum, l) => sum + l.credit, 0);
    const isBalanced = Math.abs(totalDebit - totalCredit) < 0.001;

    const submit = async () => {
        setError("");

        if (lines.length === 0) {
            return setError("Journal must have at least one line");
        }

        if (!isBalanced) {
            return setError("Journal not balanced");
        }

        try {
            await journalApi.create({
                date,
                description,
                status: "Draft",
                lines,
            });

            resetForm();
            onSuccess?.();
        } catch (err: any) {
            setError(err.message || "Failed to save journal");
        }
    };

    return (
        <div className="space-y-4 p-4 border rounded-lg max-w-4xl w-full">
            {/* Journal Type */}
            <select
                className="border p-2 rounded w-60"
                value={journalType}
                onChange={(e) => setJournalType(e.target.value as JournalType)}
            >
                <option value="manual">Manual Entry</option>
                <option value="sale">Sales</option>
                <option value="purchase">Purchase</option>
                <option value="wages">Wages</option>
            </select>

            <h2 className="text-lg font-semibold">Journal Entry</h2>

            {/* Header */}
            <div className="flex gap-2 flex-wrap">
                <input
                    type="date"
                    className="border p-2 rounded w-48"
                    value={date}
                    onChange={(e) => setDate(e.target.value)}
                />

                <input
                    className="border p-2 rounded flex-1 min-w-[200px]"
                    placeholder="Description"
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                />
            </div>

            {/* Lines */}
            <div className="border rounded-lg overflow-hidden">
                <div className="grid grid-cols-12 gap-2 bg-gray-100 p-2 text-sm font-medium">
                    <div className="col-span-5">Account</div>
                    <div className="col-span-3 text-right">Debit</div>
                    <div className="col-span-3 text-right">Credit</div>
                    <div className="col-span-1 text-center">✕</div>
                </div>

                <div className="max-h-64 overflow-y-auto">
                    {lines.map((line, i) => (
                        <div
                            key={i}
                            className="grid grid-cols-12 gap-2 p-2 border-t items-center"
                        >
                            <input
                                className="col-span-5 border p-2 rounded w-full"
                                value={line.account_id}
                                onChange={(e) =>
                                    updateLine(i, "account_id", e.target.value)
                                }
                                disabled={journalType !== "manual"}
                            />

                            <input
                                type="number"
                                className="col-span-3 border p-2 rounded text-right"
                                value={line.debit}
                                onChange={(e) =>
                                    updateLine(i, "debit", Number(e.target.value))
                                }
                            />

                            <input
                                type="number"
                                className="col-span-3 border p-2 rounded text-right"
                                value={line.credit}
                                onChange={(e) =>
                                    updateLine(i, "credit", Number(e.target.value))
                                }
                            />

                            <div className="col-span-1 text-center">
                                <button
                                    onClick={() => removeLine(i)}
                                    className="text-red-500"
                                >
                                    ✕
                                </button>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            {/* Totals */}
            <div className="flex justify-end gap-6 text-sm font-medium">
                <span>Total Debit: {totalDebit.toLocaleString()}</span>
                <span>Total Credit: {totalCredit.toLocaleString()}</span>
            </div>

            {!isBalanced && (
                <div className="text-red-500 text-sm">
                    Journal not balanced
                </div>
            )}

            {error && <div className="text-red-500 text-sm">{error}</div>}

            {/* Actions */}
            <div className="flex justify-between items-center">
                <button
                    onClick={addLine}
                    disabled={journalType !== "manual"}
                    className="bg-gray-200 px-3 py-1 rounded disabled:opacity-50"
                >
                    + Add Line
                </button>

                <div className="flex gap-2">
                    <button
                        onClick={handleCancel}
                        className="px-4 py-2 border rounded"
                    >
                        Batal
                    </button>

                    <button
                        onClick={submit}
                        className="bg-blue-500 text-white px-4 py-2 rounded"
                    >
                        Simpan Jurnal
                    </button>
                </div>
            </div>
        </div>
    );
}