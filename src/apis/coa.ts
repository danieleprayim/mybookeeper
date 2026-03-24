import { invoke } from "@tauri-apps/api/core";

export const coaApi = {
    
    create: (data: any) =>
        invoke("create_coa", data),

    
    update: (data: any) =>
        invoke("update_coa", data),

    
    setActive: (id: string, active: boolean) =>
        invoke("set_coa_active", { id, active }),

    
    list: () =>
        invoke("list_all_coa"),

};