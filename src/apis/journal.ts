import { invoke } from "@tauri-apps/api/core";

export const journalApi = {
    create: (data: any) =>
        invoke("create_journal", { dto: data }),

    post: (journal_id: string) =>
        invoke("post_journal", { journal_id }),

    list: () =>
        invoke("list_all"),

    detail: (journal_id: string) =>
        invoke("get_by_id", { journal_id }),
};