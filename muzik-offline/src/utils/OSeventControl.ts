import { Payload } from "@muziktypes/index";
import { pauseSong, playNextSong, playPreviousSong, playSong, stopSong } from "./playerControl";
import { appWindow } from '@tauri-apps/api/window';

export async function processOSMediaControlsEvent(event: Payload) {
    switch(event.event){
        case "Play": 
            playSong();
            break;
        case "Pause": 
            pauseSong();
            break;
        case "Toggle": 
            // idk what this is
            break;
        case "Next": 
            playNextSong();
            break;
        case "Previous": 
            playPreviousSong();
            break;
        case "Stop": 
            stopSong();
            break;
        case "Quit":
            appWindow.close();
            break;
        default:
            break;
    }
}