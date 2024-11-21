import { useDirStore, useSavedObjectStore, useToastStore } from "@store/index";
import "@styles/layouts/MusicFoldersSettings.scss"; 
import { motion } from "framer-motion";
import { FunctionComponent, useEffect, useState } from "react";
import { open } from '@tauri-apps/plugin-dialog';
import { appConfigDir } from '@tauri-apps/api/path';
import { Cross } from "@assets/icons";
import { local_songs_db, local_albums_db, local_artists_db, local_genres_db } from "@database/database";
import { toastType } from "@muziktypes/index";
import { invoke } from "@tauri-apps/api/core";
import { isPermissionGranted, sendNotification } from "@tauri-apps/plugin-notification";
import { areArraysDifferent, fetch_library } from "@utils/index";

type MusicFoldersSettingsProps = {
    openConfirmModal: (path: string) => void;
}

const MusicFoldersSettings: FunctionComponent<MusicFoldersSettingsProps> = (props: MusicFoldersSettingsProps) => {
    const { dir, setDir } = useDirStore((state) => { return { dir: state.dir, setDir: state.setDir}; });
    const [oldDir] = useState<string[]>(dir.Dir);
    const { setToast } = useToastStore((state) => { return { setToast: state.setToast }; });
    const {local_store} = useSavedObjectStore((state) => { return { local_store: state.local_store}; });
    const [directory, setDirectory] = useState<string>("");

    function reloadSongs(){
        invoke("get_all_songs", { pathsAsJsonArray: JSON.stringify(dir.Dir), compressImageOption: local_store.CompressImage === "Yes" ? true : false })
        .then(async() => {
                await local_songs_db.songs.clear();
                await local_albums_db.albums.clear();
                await local_artists_db.artists.clear();
                await local_genres_db.genres.clear();
                const res = await fetch_library(true);
                let message = "";

                if(res.status === "error")message = res.message;
                else message = "Successfully loaded all the songs in the paths specified. You may need to reload the page you are on to see your new songs";

                setToast({title: "Loading songs...", message: message, type: toastType.success, timeout: 5000});

                const permissionGranted = await isPermissionGranted();
                if(permissionGranted)sendNotification({ title: 'Loading songs...', body: message });
            })
            .catch(async(_error) => {
                console.log(_error);
                setToast({title: "Loading songs...", message: "Failed to load all the songs in the paths specified", type: toastType.error, timeout: 5000});
                const permissionGranted = await isPermissionGranted();
                if (permissionGranted) {
                    sendNotification({ title: 'Loading songs...', body: 'Failed to load all the songs in the paths specified' });
                }
            });
    }

    async function openFileDialog(){
        const selected = await open({
            directory: true,
            multiple: false,
            defaultPath: await appConfigDir(),
        });
        if(selected) setDir({Dir: dir.Dir.concat(selected)});
    };

    function addNewDir(){
        if(directory === "")return;
        setDir({Dir: dir.Dir.concat(directory)});
        setDirectory("");
    }

    useEffect(() => {
        // when component unmounts
        return () => {
            if(areArraysDifferent(oldDir, dir.Dir)){
                setToast({title: "Loading songs...", message: "We are searching for new songs", type: toastType.warning, timeout: 5000});
                reloadSongs();
            }
        }
    }, [dir.Dir]);

    
    return (
        <div className="MusicFoldersSettings">
            <h2>Music Folder Settings</h2>
            <div className="header">
                <input type="text" placeholder="Type a folder path" value={directory} onChange={(e) => setDirectory(e.target.value)}/>
                <div className="buttons">
                    <motion.div className="add_directory" whileTap={{scale: 0.98}} onClick={addNewDir}>
                        <h3>Add directory</h3>
                    </motion.div>
                    <motion.div className="select_directory" whileTap={{scale: 0.98}} onClick={openFileDialog}>
                        <h3>Select a directory</h3>
                    </motion.div>
                </div>
            </div>
            <div className="MusicFoldersSettings_container">
                {
                    dir.Dir.map((value, index) => 
                        <div className="path" key={index}>
                            <h3>{value}</h3>
                            <div className="icon" onClick={() => props.openConfirmModal(value)}>
                                <Cross/>
                            </div>
                        </div>
                    )
                }
            </div>
        </div>
    )
}

export default MusicFoldersSettings