import { useState } from "react";
import { journalApi } from "../../apis/journal";

export default function JournalForm({ onSuccess }: any) {
    const [date, setDate] = useState("");
    const [description, setDescription] = useState("");

    const submit = async () => {
        await journalApi.create({
            date,
            description,
            lines: [], // next step: dynamic lines
        });

        setDate("");
        setDescription("");
        onSuccess();
    };

    return (
        <div className="flex gap-2">
            <input
                type="date"
                className="border p-2"
                value={date}
                onChange={(e) => setDate(e.target.value)}
            />

            <input
                className="border p-2"
                placeholder="Description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
            />

            <button onClick={submit} className="bg-blue-500 text-white px-4">
                Add Journal
            </button>
        </div>
    );
}