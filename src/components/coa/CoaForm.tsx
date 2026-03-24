import { useState } from "react";
import { coaApi } from "../../apis/coa";

function Field({ label, children }: any) {
    return (
        <div className="grid grid-cols-3 items-center gap-3">
            <label className="text-sm text-gray-600">{label}</label>
            <div className="col-span-2">{children}</div>
        </div>
    );
}

function Section({ title, children }: any) {
    return (
        <div>
            <h3 className="text-sm font-semibold text-gray-500 mb-2">
                {title}
            </h3>
            <div className="flex flex-col gap-3">{children}</div>
        </div>
    );
}

export default function CoaForm({ onSuccess, onCancel }: any) {
    const [form, setForm] = useState({
        account_code: "",
        account_name: "",
        account_type: "NOMINAL",
        account_group_code: "",
        account_group_name: "",
        normal_account: "DEBET",
        description: "",
        is_active: true,
        parent_id: "",
    });

    const GROUP_OPTIONS = [
        { label: "Asset", value: "ASSET" },
        { label: "Liability", value: "LIABILITY" },
        { label: "Equity", value: "EQUITY" },
        { label: "Income", value: "INCOME" },
        { label: "Expense", value: "EXPENSE" },
    ];

    const setField = (key: string, value: any) => {
        setForm((prev) => ({ ...prev, [key]: value }));
    };

    const submit = async () => {
        
        try {
            if (!form.account_code || !form.account_name) {
                return alert("Code & Name required");
            }

            console.log("SUBMIT DATA:", form);

            await coaApi.create({dto: form});

            onSuccess();
        } catch (err) {
            console.error(err);
            alert("Failed to save COA");
        }
    };

    return (
        <div className="flex flex-col gap-4">

            {/* GROUP */}
            <Section title="Group">
                <Field label="Account Group">
                    <select
                        className="input"
                        value={form.account_group_code}
                        onChange={(e) => {
                            const value = e.target.value;

                            setForm((prev) => ({
                                ...prev,
                                account_group_code: value,
                                account_group_name: value,
                            }));
                        }}
                    >
                        <option value="">-- Select Group --</option>
                        {GROUP_OPTIONS.map((g) => (
                            <option key={g.value} value={g.value}>
                                {g.label}
                            </option>
                        ))}
                    </select>
                </Field>
            </Section>

            {/* CLASSIFICATION */}
            <Section title="Classification">
                <Field label="Account Type">
                    <select
                        className="input"
                        value={form.account_type}
                        onChange={(e) => setField("account_type", e.target.value)}
                    >
                        <option value="NOMINAL">Nominal</option>
                        <option value="REAL">Real</option>
                    </select>
                </Field>

                <Field label="Normal Account">
                    <select
                        className="input"
                        value={form.normal_account}
                        onChange={(e) => setField("normal_account", e.target.value)}
                    >
                        <option value="DEBET">Debet</option>
                        <option value="CREDIT">Credit</option>
                    </select>
                </Field>
            </Section>

            {/* BASIC */}
            <Section title="Basic Info">
                <Field label="Account Code">
                    <input
                        className="input"
                        value={form.account_code}
                        onChange={(e) => setField("account_code", e.target.value)}
                    />
                </Field>

                <Field label="Account Name">
                    <input
                        className="input"
                        value={form.account_name}
                        onChange={(e) => setField("account_name", e.target.value)}
                    />
                </Field>
            </Section>

            {/* EXTRA */}
            <Section title="Additional">
                <Field label="Description">
                    <textarea
                        className="input"
                        value={form.description}
                        onChange={(e) => setField("description", e.target.value)}
                    />
                </Field>

                <Field label="Active">
                    <input
                        type="checkbox"
                        checked={form.is_active}
                        onChange={(e) => setField("is_active", e.target.checked)}
                    />
                </Field>
            </Section>

            {/* ACTION */}
            <div className="flex justify-end gap-2 mt-4">
                <button onClick={onCancel} className="btn-secondary">
                    Batal
                </button>
                <button type="button" onClick={submit} className="btn-primary">
                    Simpan
                </button>
            </div>
        </div>
    );
}