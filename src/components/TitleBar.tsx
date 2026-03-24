import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";

export default function TitleBar() {
    const appWindow = getCurrentWindow();

    return (
        <div className="flex items-center justify-between bg-gray-900 text-white px-4 py-2 select-none">

            {/* Drag ONLY here */}
            <div data-tauri-drag-region className="flex-1">
                MyBookeeper v.0.0.1
            </div>

            {/* NOT draggable */}
            <div className="flex gap-2">
                <button
                    data-tauri-drag-region={false}
                    onClick={() => appWindow.minimize()}
                    className="p-2 hover:bg-gray-700 rounded"
                >
                    <Minus size={18} />
                </button>

                <button
                    data-tauri-drag-region={false}
                    onClick={() => appWindow.toggleMaximize()}
                    className="p-2 hover:bg-gray-700 rounded"
                >
                    <Square size={18} />
                </button>

                <button
                    data-tauri-drag-region={false}
                    onClick={() => appWindow.close()}
                    className="p-2 hover:bg-red-600 rounded"
                >
                    <X size={18} />
                </button>
            </div>
        </div>
    );
}