import { createContext, useContext, useState, useEffect } from "react";
import type { ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";

interface AuthContextType {
    isAuthenticated: boolean;
    loading: boolean;
    login: (email: string, password: string) => Promise<boolean>;
    logout: () => Promise<void>;
    validateSession: () => Promise<boolean>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
    
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [loading, setLoading] = useState(true);

    useEffect(() => {

        const token = localStorage.getItem("auth_token");
        
        if (!token) {
            setLoading(false);
            return;
        }
        
        invoke("get_session", { token })
            .then(() => setIsAuthenticated(true))
            .catch(() => localStorage.removeItem("auth_token"))
            .finally(() => setLoading(false));
    }, []);

    const login = async (email: string, password: string) => {
        
        try {
            const token = await invoke<string>("login", { dto: { email, password } });
            localStorage.setItem("auth_token", token);
            setIsAuthenticated(true);
            return true;
        } catch (err) {
            console.error("Login failed:", err);
            setIsAuthenticated(false);
            return false;
        }
    };

    const logout = async () => {
        
        const token = localStorage.getItem("auth_token");
        
        if (token) {
            await invoke("logout", { token }).catch(() => { });
            localStorage.removeItem("auth_token");
        }
        
        setIsAuthenticated(false);
    };

    const validateSession = async () => {
        const token = localStorage.getItem("auth_token");
        
        if (!token) {
            return false;
        }

        try {
            await invoke("get_session", { token });
            setIsAuthenticated(true);
            return true;
        } catch {
            localStorage.removeItem("auth_token");
            setIsAuthenticated(false);
            return false;
        }
    };

    return (
        <AuthContext.Provider value={{ isAuthenticated, loading, login, logout, validateSession }}>
            {children}
        </AuthContext.Provider>
    );
};

export const useAuth = () => {
    
    const context = useContext(AuthContext);
    
    if (!context) {
        throw new Error("useAuth must be used within AuthProvider")
    };
    
    return context;
};