import { invoke } from "@tauri-apps/api/core";

export const login = async (email: string, password: string) => {
    try {
        const token = await invoke<string>("login", { dto: { email, password } });
        console.log("TOKEN:", token);
        localStorage.setItem("auth_token", token);
        return token;
    } catch (err) {
        console.error("LOGIN ERROR:", err);
        return null;
    }
};

export const logout = async () => {
    const token = localStorage.getItem("auth_token");
    if (!token) return;
    await invoke("logout", { token });
    localStorage.removeItem("auth_token");
};

export const validateSession = async () => {
    const token = localStorage.getItem("auth_token");
    if (!token) {
        return false
    }
    
    try {
        await invoke("get_session", { token });
        return true;
    } catch {
        localStorage.removeItem("auth_token");
        return false;
    }
};