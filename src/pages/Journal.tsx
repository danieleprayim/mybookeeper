import { useEffect, useState } from "react";
import { journalApi } from "../apis/journal";
import JournalTable from "../components/journal/JournalTable";
import JournalForm from "../components/journal/JournalForm";
import Sidebar from "../components/Sidebar";

export default function JournalPage() {

    const [data, setData] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [open, setOpen] = useState(false);

    const load = async () => {
        try {
            setLoading(true);
            const res = await journalApi.list();
            setData(res as any[]);
        } catch (error) {
            console.error(error);
            alert(error?.toString() || "Unknown error");
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        load();
    }, []);

    return (
        <div className="flex h-screen bg-gray-100">
            
            <Sidebar />

            <main className="flex-1 p-6 overflow-auto">

                {/* HEADER */}
                <div className="flex items-center justify-between mb-6">
                    <h1 className="text-2xl font-semibold text-gray-800">
                        Daftar Jurnal Umum
                    </h1>

                    <button
                        onClick={() => setOpen(true)}
                        className="bg-blue-600 text-white px-4 py-2 rounded-lg shadow hover:bg-blue-700 transition"
                    >
                        + Tambah Transaksi
                    </button>
                </div>

                {/* TABLE CARD */}
                <div className="bg-white rounded-xl shadow p-4">

                    {loading ? (
                        <div className="p-4 text-gray-500">Loading...</div>
                    ) : data.length === 0 ? (
                        <div className="p-4 text-gray-500">
                            Belum ada transaksi yang dicatat
                        </div>
                    ) : (
                        <JournalTable data={data} />
                    )}

                </div>

                {/* MODAL */}
                {open && (
                    <div className="fixed inset-0 bg-black/40 flex items-center justify-center z-50">
                        <div className="bg-white rounded-xl shadow-lg w-[500px] p-6">

                            <h2 className="text-lg font-semibold mb-4">
                                Catat Transaksi
                            </h2>

                            <JournalForm
                                onSuccess={async () => {
                                    setOpen(false);
                                    await load(); // ensure fresh data
                                }}
                                onCancel={() => setOpen(false)}
                            />
                        </div>
                    </div>
                )}
            </main>
            
        </div>
    );
}