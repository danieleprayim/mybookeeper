import accountantgirl from "../assets/accountantgirl.svg";
import calculator3 from "../assets/calculator3.svg";
import Button from "../components/Button";
import { useNavigate } from "react-router-dom";

const Landing = () => {
    
    const navigate = useNavigate();

    return (
        
        <div className="h-screen bg-gray-50 flex flex-col overflow-hidden">

            <header className="h-16 flex items-center justify-between px-8 border-b bg-white">
                
                <div className="flex items-center gap-3 cursor-pointer">
                    <img
                        src={calculator3}
                        alt="Logo"
                        className="w-8 h-8"
                    />
                    <span className="text-2xl font-semibold">
                        <span className="text-blue-600">My</span>
                        <span className="text-gray-900">Boo</span>
                        <span className="text-gray-600">Keeper</span>
                    </span>
                </div>

                <div className="flex items-center gap-4">
                    <button
                        onClick={() => navigate("/login")}
                        className="text-sm text-gray-600 hover:text-blue-600"
                    >
                        Login
                    </button>

                    <Button onClick={() => navigate("/login")}>
                        Get Started
                    </Button>
                </div>
            </header>

            <main className="flex-1 flex items-center justify-center px-8">
                <div className="w-full max-w-6xl grid grid-cols-2 gap-12 items-center">

                    {/* Left Content */}
                    <div>
                        <h1 className="text-5xl font-extrabold text-gray-900 leading-tight">
                            Lightweight Desktop <br />
                            Bookeeping System
                        </h1>

                        <p className="mt-6 text-lg text-gray-600 leading-relaxed">
                            This is beta version App designed to manage your transactions, reports, and financial workflows
                            in one powerful desktop application built for SMEs.
                        </p>

                        <div className="mt-8 flex items-center gap-4">

                            <button
                                className="text-gray-600 hover:text-blue-600 text-sm"
                            >
                                Learn More →
                            </button>
                        </div>

                        {/* Optional stats (adds production feel) */}
                        <div className="mt-10 flex gap-8 text-sm text-gray-500">
                            <div>
                                <p className="text-xl font-bold text-gray-800">10K+</p>
                                <p>Transactions</p>
                            </div>
                            <div>
                                <p className="text-xl font-bold text-gray-800">500+</p>
                                <p>Businesses</p>
                            </div>
                            <div>
                                <p className="text-xl font-bold text-gray-800">99.9%</p>
                                <p>Reliability</p>
                            </div>
                        </div>
                    </div>

                    {/* Right Content (Image) */}
                    <div className="flex justify-center">
                        <div className="relative">

                            {/* Glow background */}
                            <div className="absolute inset-0 bg-blue-200 blur-3xl opacity-30 rounded-full"></div>

                            {/* Image */}
                            <img
                                src={accountantgirl}
                                alt="Accounting illustration"
                                className="relative w-full max-w-sm"
                            />
                        </div>
                    </div>
                </div>
            </main>

            {/* ===== Footer ===== */}
            <footer className="h-10 flex items-center justify-center text-xs text-gray-400">
                © 2026 MyBooKeeper. All rights reserved.
            </footer>
        </div>
    );
};

export default Landing;