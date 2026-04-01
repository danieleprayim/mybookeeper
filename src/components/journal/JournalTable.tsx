import { useState } from "react";

type JournalLine = {
    id: string;
    account_id: string;
    account_name: string;
    debit: number;
    credit: number;
};

type Journal = {
    id: string;
    date: string;
    description: string;
    status: "Draft" | "Posted";
    lines: JournalLine[];
};

type Props = {
    data: Journal[];
};

export default function JournalTable({ data }: Props) {
    const [openId, setOpenId] = useState<string | null>(null);

    return (
        <table className="w-full text-sm border">
            <thead className="bg-gray-100 text-left">
                <tr>
                    <th className="p-2">Tanggal</th>
                    <th className="p-2">Deskripsi</th>
                    <th className="p-2 text-right">Debit</th>
                    <th className="p-2 text-right">Kredit</th>
                    <th className="p-2">Status</th>
                </tr>
            </thead>

            <tbody>
                {data.map((j) => {
                    const totalDebit = j.lines.reduce((a, b) => a + b.debit, 0);
                    const totalCredit = j.lines.reduce((a, b) => a + b.credit, 0);

                    return (
                        <>
                            {/* MAIN ROW */}
                            <tr
                                key={j.id}
                                className="border-b hover:bg-gray-50 cursor-pointer"
                                onClick={() =>
                                    setOpenId(openId === j.id ? null : j.id)
                                }
                            >
                                <td className="p-2">{j.date}</td>
                                <td className="p-2">{j.description}</td>
                                <td className="p-2 text-right">
                                    {totalDebit.toLocaleString()}
                                </td>
                                <td className="p-2 text-right">
                                    {totalCredit.toLocaleString()}
                                </td>
                                <td className="p-2">
                                    <span
                                        className={`px-2 py-1 rounded text-xs ${j.status === "Posted"
                                                ? "bg-green-100 text-green-700"
                                                : "bg-yellow-100 text-yellow-700"
                                            }`}
                                    >
                                        {j.status}
                                    </span>
                                </td>
                            </tr>

                            {/* DETAIL ROW */}
                            {openId === j.id && (
                                <tr className="bg-gray-50">
                                    <td colSpan={5}>
                                        <table className="w-full text-sm">
                                            <thead>
                                                <tr className="text-gray-500">
                                                    <th className="p-2 text-left">
                                                        Akun
                                                    </th>
                                                    <th className="p-2 text-right">
                                                        Debit
                                                    </th>
                                                    <th className="p-2 text-right">
                                                        Kredit
                                                    </th>
                                                </tr>
                                            </thead>

                                            <tbody>
                                                {j.lines.map((l) => (
                                                    <tr key={l.id}>
                                                        <td className="p-2">
                                                            {l.account_name}
                                                        </td>
                                                        <td className="p-2 text-right">
                                                            {l.debit > 0
                                                                ? l.debit.toLocaleString()
                                                                : "-"}
                                                        </td>
                                                        <td className="p-2 text-right">
                                                            {l.credit > 0
                                                                ? l.credit.toLocaleString()
                                                                : "-"}
                                                        </td>
                                                    </tr>
                                                ))}
                                            </tbody>
                                        </table>
                                    </td>
                                </tr>
                            )}
                        </>
                    );
                })}
            </tbody>
        </table>
    );
}