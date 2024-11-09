import { modal_variants } from "@content/index";
import { SavedObject } from "@database/saved_object";
import { useSavedObjectStore, useWallpaperStore } from "@store/index";
import { motion } from "framer-motion";
import { FunctionComponent, useState } from "react";
import "@styles/components/modals/WallpapersSelectionModal.scss";

type WallpapersSelectionModalProps = {
    isOpen: boolean;
    closeModal: () => void;
}

const WallpapersSelectionModal: FunctionComponent<WallpapersSelectionModalProps> = (props: WallpapersSelectionModalProps) => {
    const {local_store, setStore} = useSavedObjectStore((state) => { return { local_store: state.local_store, setStore: state.setStore}; });
    //const { wallpaperUUID, setWallpaper } = useWallpaperStore((state) => { return { wallpaperUUID: state.wallpaperUUID, setWallpaper: state.setWallpaper, unsetWallpaper: state.unsetWallpaper }; });
    const [wallpapers, setWallpapers] = useState<string[]>([
        "blue_purple_gradient",
        "blue_purple_gradient",
        "blue_purple_gradient",
        "blue_purple_gradient",
        "blue_purple_gradient",
        "blue_purple_gradient",
        "blue_purple_gradient",
    ]);

    function uploadImg(_e: any){
        //const image = e.target.files[0];
        //const reader = new FileReader();
        //reader.readAsDataURL(image);
        
        //reader.addEventListener('load', () => setWallpaper({DisplayWallpaper: reader.result}));
        let temp: SavedObject = local_store;
        temp.BGColour = "";
        setStore(temp);
    }

    return (
        <div className={"WallpapersSelectionModal" + (props.isOpen ? " WallpapersSelectionModal-visible" : "")} onClick={
            (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {if(e.target === e.currentTarget)props.closeModal();}}>
            <motion.div 
            animate={props.isOpen ? "open" : "closed"}
            variants={modal_variants}
            className="modal">
                <h2>Add or Choose a wallpaper</h2>

                <div className="wallpapers">
                    <motion.label className="wallpaper add_wallpaper " whileHover={{scale: 1.03}} whileTap={{scale: 0.98}}>
                            <input name="background-img" type="file" accept="image/png, image/jpeg" onChange={uploadImg}/>
                            add wallpaper
                    </motion.label>
                    {wallpapers.map((wallpaper, index) => {
                        return (
                            <motion.div 
                            className="wallpaper"
                            whileHover={{scale: 1.03}} 
                            whileTap={{scale: 0.98}} 
                            onClick={() => {}} key={index}>
                                <h4>{wallpaper}</h4>
                            </motion.div>
                        )
                    })}
                </div>

            </motion.div>
        </div>
    )
}

export default WallpapersSelectionModal