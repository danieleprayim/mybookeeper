import { useNavigate, NavLink } from "react-router-dom";
import calculator3 from "../assets/calculator3.svg";

const Sidebar = () => {
    
    const navigate = useNavigate();

    const handleLogout = () => {
        localStorage.removeItem("auth_token");
        navigate("/login");
    };

    const menuItems = [
        { name: "Dashboard", path: "/dashboard" },
        { name: "Accounts", path: "/accounts" },
        { name: "Transactions", path: "/transactions" },
        { name: "Customers", path: "/dashboard/cistomers" },
        { name: "Reports", path: "/dashboard/reports" },
        { name: "Settings", path: "/dashboard/settings" },
    ];

    return (
        <aside className="w-64 bg-white border-r h-screen flex flex-col">

            <div className="h-16 flex items-center px-6 border-b">
                <img
                    src={calculator3}
                    alt="Logo"
                    className="w-8 h-8"
                />
                <span className="text-xl font-semibold">
                    <span className="text-blue-600">My</span>
                    <span className="text-gray-800">BooK</span>
                    <span className="text-gray-400">eeper</span>
                </span>
            </div>

            <nav className="flex-1 px-3 py-6 space-y-1">
                {menuItems.map((item) => (
                    <NavLink
                        key={item.name}
                        to={item.path}
                        className={({ isActive }) =>
                            `flex items-center px-4 py-2 rounded-lg  transition
                            ${isActive
                                ? "bg-blue-50 text-blue-600 border-l-4 border-blue-600 text-medium font-semibold"
                                : "text-gray-600 hover:bg-gray-100 hover:text-gray-900 text-medium font-medium"
                            }`
                        }
                    >
                        {item.name}
                    </NavLink>
                ))}
            </nav>

            <div className="p-4 border-t">
                <button
                    onClick={handleLogout}
                    className="w-full flex items-center justify-center px-4 py-2 text-sm font-medium text-red-500 hover:bg-red-50 rounded-lg transition"
                >
                    Logout
                </button>
            </div>
        </aside>
    );
};

export default Sidebar;