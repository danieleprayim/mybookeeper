import { invoke } from "@tauri-apps/api/core";

export const journalApi = {

    create: (data: any) =>
        invoke("create_journal", data),

    
    post: (journalId: string, token: string) =>
        invoke("post_journal", { journalId, token }),

    
    list: () =>
        invoke("list_journal"),

    
    detail: (id: string) =>
        invoke("get_journal", { id }),
    
};