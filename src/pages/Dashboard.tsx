import Sidebar from "../components/Sidebar";

const Dashboard = () => {
    return (
        <div className="flex h-screen bg-gray-100">
            
            <Sidebar />
            
            <main className="flex-1 p-8 overflow-auto">
                
                <h1 className="text-3xl font-bold text-gray-800 mb-6">
                    Dashboard
                </h1>
                
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">

                    <div className="bg-white p-6 rounded-xl shadow hover:shadow-lg transition">
                        <h2 className="text-lg font-semibold text-gray-700">
                            Total Accounts
                        </h2>
                        
                        <p className="text-2xl font-bold mt-2">
                            12
                        </p>
                    </div>
                    
                    <div className="bg-white p-6 rounded-xl shadow hover:shadow-lg transition">
                        <h2 className="text-lg font-semibold text-gray-700">
                            Transactions
                        </h2>
                        <p className="text-2xl font-bold mt-2">
                            234
                        </p>
                    </div>
                    
                    <div className="bg-white p-6 rounded-xl shadow hover:shadow-lg transition">
                        <h2 className="text-lg font-semibold text-gray-700">
                            Reports
                        </h2>
                        <p className="text-2xl font-bold mt-2">
                            5
                        </p>
                    </div>

                </div>
            </main>
            
        </div>
    );
};

export default Dashboard;