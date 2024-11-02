import { Payload } from "@muziktypes/index";
import { changeSeekerPosition, changeSeekerPositionBtnPress, changeVolumeLevel, pauseSong, playNextSong, playPreviousSong, playSong, setVolumeLevel, stopSong } from "./playerControl";
import { appWindow } from '@tauri-apps/api/window';
import { invoke } from "@tauri-apps/api";
import { usePlayerStore, usePlayingPositionSec } from "@store/index";

export async function processOSMediaControlsEvent(event: Payload) {
    const song = usePlayerStore.getState().Player.playingSongMetadata;
    const position_sec = usePlayingPositionSec.getState().position;
    if (!song) return;
    switch(event.event){
        case "Play": 
            playSong();
            break;
        case "Pause": 
            pauseSong();
            break;
        case "Toggle": 
            // what is this
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
        case "Seek":
            if(event.seek_direction === "forward"){
                changeSeekerPositionBtnPress(false);
            }else{
                changeSeekerPositionBtnPress(true);
            }
            break;
        case "SeekBy":
            if(event.seek_direction === "forward"){
                changeSeekerPosition((position_sec + (event.duration ?? 0)) * song.duration_seconds);
            }else{
                changeSeekerPosition((position_sec + (event.duration ?? 0)) * song.duration_seconds);
            }
            break;
        case "SetPosition":
            break;
        case "SetVolume":
            // set volume
            if(event.volume){
                changeVolumeLevel(event.volume * 100);
                setVolumeLevel(event.volume * 100);
            }
            break;
        case "OpenUri":
            // open the uri
            await invoke("open_in_file_manager", { filePath: song.path });
            break;
        case "Raise":
            // bring the app to the front
            await appWindow.setFocus();
            break;
        case "Quit":
            // close the app
            await appWindow.close();
            break;
        default:
            break;
    }
}