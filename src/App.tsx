import {useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";

import {open} from '@tauri-apps/api/dialog';

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
        <div className="container">
            <h1>Welcome to the injector</h1>

            <p>Enter a process name & select your dll.</p>

            <div className="row">
                <div>
                    <input
                        id="injection"
                        onChange={(e) => setProcess(e.currentTarget.value)}
                        placeholder="Enter a process name"
                    />
                    <button type="button" onClick={() => openFile()}>
                        Select dll
                    </button>
                    <button type="button" onClick={() => inject()}>
                        Inject
                    </button>
                </div>
            </div>
        </div>
    );
}

export default App;
