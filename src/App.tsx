import { useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Landing from "./pages/Landing";
import Login from "./pages/Login";
import Dashboard from "./pages/Dashboard";
import ProtectedRoute from "./routes/ProtectedRoute";
import "./tailwind.css";
import CoaPage from "./pages/Coa";
import JournalPage from "./pages/Journal";
// import TitleBar from "./components/TitleBar";

function App() {
  useEffect(() => {
    getCurrentWindow().maximize().catch(() => { });
  }, []);

  return (
    
    <BrowserRouter>

      <Routes>
        {/* Public pages */}
        <Route path="/" element={<Landing />} />
        
        <Route path="/login" element={<Login />} />

        {/* Protected pages */}
        <Route
          path="/dashboard"
          element={
            <ProtectedRoute>
              <Dashboard />
            </ProtectedRoute>
          }
        />

        <Route
          path="/accounts"
          element={
            <ProtectedRoute>
              <CoaPage />
            </ProtectedRoute>
          }
        />

        <Route
          path="/transactions"
          element={
            <ProtectedRoute>
              <JournalPage />
            </ProtectedRoute>
          }
        />
      </Routes>
    </BrowserRouter>
  );
}
export default App;