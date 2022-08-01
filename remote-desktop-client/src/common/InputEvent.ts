import { invoke } from "@tauri-apps/api/tauri";

import { Command } from "./Constans"

const handleMouseEvent = async (data: any) => {
    await invoke(Command.MOUSE_EVENT, data);
};

const handleKeyboardEvent = async (data: any) => {
    await invoke(Command.KEY_EVENT, data);
};

export { handleMouseEvent, handleKeyboardEvent }