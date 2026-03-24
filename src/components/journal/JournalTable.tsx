import { journalApi } from "../../apis/journal";

export default function JournalTable({ data, onRefresh }: any) {
    const post = async (id: string) => {
        const token = localStorage.getItem("token") || "";
        await journalApi.post(id, token);
        onRefresh();
    };

    return (
        <table className="w-full mt-4 border">
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Description</th>
                    <th>Status</th>
                    <th></th>
                </tr>
            </thead>

            <tbody>
                {data.map((j: any) => (
                    <tr key={j.id}>
                        <td>{j.date}</td>
                        <td>{j.description}</td>
                        <td>{j.status}</td>
                        <td>
                            {j.status === "Draft" && (
                                <button
                                    onClick={() => post(j.id)}
                                    className="bg-green-500 text-white px-2"
                                >
                                    Post
                                </button>
                            )}
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}