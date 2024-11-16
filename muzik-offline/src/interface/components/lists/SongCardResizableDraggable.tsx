import { FunctionComponent } from "react";
import { Song } from "@muziktypes/index";
import { SongCardResizable } from "@components/index";
import { Reorder } from "framer-motion";

type SongCardResizableDraggableProps = {
    SongQueue: Song[];
    queueType: "SongQueue" | "SongHistory";
    onDragEnd: (reordered: Song[], queueType: "SongQueue" | "SongHistory") => void;
    setMenuOpenData: (key: number, co_ords: {xPos: number; yPos: number;}) => void;
    playThisSong: (key: number, index: number, queueType: "SongQueue" | "SongHistory") => void;
    navigateTo: (key: number, type: "artist" | "song", queueType: "SongQueue" | "SongHistory") => void;
}

const SongCardResizableDraggable: FunctionComponent<SongCardResizableDraggableProps> = (props: SongCardResizableDraggableProps) => {
    return (
        <Reorder.Group 
        values={props.SongQueue} 
        onReorder={(newOrder: Song[]) => props.onDragEnd(newOrder, props.queueType)}
        axis="y"
        as="div"
        layoutScroll
        style={{ overflowY: "scroll"}}>
                {props.SongQueue.map((song, index) =>
                        <Reorder.Item value={song} key={song.id} as="div">
                            <SongCardResizable 
                                key={index}
                                cover={song.cover_uuid} 
                                songName={song.name}
                                artist={song.artist}
                                keyV={song.id}
                                setMenuOpenData={props.setMenuOpenData}
                                playThisSong={(key: number) => props.playThisSong(key, index, props.queueType)}
                                navigateTo={(key: number, type: "artist" | "song") => props.navigateTo(key, type, "SongQueue")}/>
                        </Reorder.Item>
                    )
                }
    </Reorder.Group>
    )
}

export default SongCardResizableDraggable