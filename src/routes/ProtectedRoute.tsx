import { Navigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";
import type { JSX } from "react";

interface ProtectedRouteProps {
  children: JSX.Element;
}

const ProtectedRoute = ({ children }: ProtectedRouteProps) => {
  const { isAuthenticated, loading } = useAuth();

  // While checking session, show nothing or a spinner
  if (loading) return <div className="flex justify-center items-center h-screen">Loading...</div>;

  // If not authenticated, redirect to login
  if (!isAuthenticated) return <Navigate to="/login" replace />;

  // Authenticated, render children
  return children;
};

export default ProtectedRoute;