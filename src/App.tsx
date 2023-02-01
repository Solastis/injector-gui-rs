import {useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";
import {appWindow} from '@tauri-apps/api/window'
import {open} from '@tauri-apps/api/dialog';
import {CgCloseR, CgMaximize, CgMinimizeAlt} from "react-icons/all";

function App() {

    const [dll, setDll] = useState("");
    const [process, setProcess] = useState("");

    async function openFile() {
        const selected = await open({
            multiple: true,
            filters: [{
                name: 'DLLs',
                extensions: ['dll']
            }]
        });
        if (selected != null) setDll(selected[0])
    }

    async function inject() {
        const res = await invoke('inject', {
            process: process,
            dllPath: dll
        });
        console.log(res);
    }

    return (
        <>
            <div className="titlebar">
                <p className={"titlebar-name-left"}>Sola Injector</p>
                <div data-tauri-drag-region className={"titlebar-buttons"}>
                    <div className="titlebar-button" id="titlebar-minimize" onClick={() => appWindow.minimize()}>
                        <CgMinimizeAlt/>
                    </div>
                    <div className="titlebar-button" id="titlebar-maximize"  onClick={() => appWindow.toggleMaximize()}>
                        <CgMaximize/>
                    </div>
                    <div className="titlebar-button" id="titlebar-close" onClick={() => appWindow.close()}>
                        <CgCloseR/>
                    </div>
                </div>
            </div>
            <div className="container">


                <input
                    id="injection"
                    onChange={(e) => setProcess(e.currentTarget.value)}
                    placeholder="Enter a process name"/>

                <button className={"dll"} type="button" onClick={() => openFile()}>
                    Select dll
                </button>

                <button className={"inject"} type="button" onClick={() => inject()}>
                    Inject
                </button>
            </div>
            <div className={"bottom"}>
                Made with &#128150; by <a target={"_blank"} href={"https://github.com/Solastis"}>Solastis</a>
            </div>
        </>

    );
}

export default App;
