type Coa = {
    account_id: string;
    account_code: string;
    account_name: string;
    account_type: string;
    normal_account: string;
    account_group_name: string;
    is_active: boolean;
};

type Props = {
    data: Coa[];
};

export default function CoaTable({ data }: Props) {
    if (!data || data.length === 0) {
        return (
            <div className="p-4 text-gray-500">
                Belum ada data akun
            </div>
        );
    }

    return (
        <table className="w-full text-sm">
            <thead className="text-left bg-gray-100">
                <tr>
                    <th className="p-2">Code</th>
                    <th className="p-2">Name</th>
                    <th className="p-2">Type</th>
                    <th className="p-2">Normal</th>
                    <th className="p-2">Group</th>
                    <th className="p-2">Active</th>
                </tr>
            </thead>

            <tbody>
                {data.map((item) => (
                    <tr
                        key={item.account_id}
                        className="border-b hover:bg-gray-50"
                    >
                        <td className="p-2">{item.account_code}</td>
                        <td className="p-2">{item.account_name}</td>
                        <td className="p-2">{item.account_type}</td>
                        <td className="p-2">{item.normal_account}</td>
                        <td className="p-2">{item.account_group_name}</td>
                        <td className="p-2">
                            {item.is_active ? "YA" : "TIDAK"}
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}