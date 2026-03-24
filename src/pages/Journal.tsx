import { useEffect, useState } from "react";
import { journalApi } from "../apis/journal";
import JournalTable from "../components/journal/JournalTable";
import JournalForm from "../components/journal/JournalForm";
import Sidebar from "../components/Sidebar";

export default function JournalPage() {
    const [data, setData] = useState([]);

    const load = async () => {
        const res = await journalApi.list();
        setData(res as any);
    };

    useEffect(() => {
        load();
    }, []);

    return (
        <div className="flex h-screen bg-gray-100">

            <Sidebar />

            <main className="flex-1 p-8 overflow-auto">
                
                <h1 className="text-3xl font-bold text-gray-800 mb-6">
                    Journal
                </h1>

                <JournalForm onSuccess={load} />

                <JournalTable data={data} onRefresh={load} />

            </main>

        </div>
    );
}