workers = [];

function exit() {
    for (var worker of workers) {
        worker.terminate();
    }
    console.clear();
}

async function main() {
    let fetchingSource = fetch('bin/webhogg-wasm.wasm');

    let canvasElement = document.getElementById('c');
    let offCanvas = canvasElement.transferControlToOffscreen();

    let fetchedSource = await fetchingSource;
    source = await fetchedSource.arrayBuffer();

    const modules = [
        { type: 'graphics',
            source: source,
            canvas: offCanvas,
            dt: 16 },
        { type: 'logic',
            source: source,
            canvas: [],
            dt: 10000 },
    ];
    for (var module of modules) {
        let worker = new Worker('pkg/worker.js');
        if (module.type === 'graphics') {
            worker.postMessage(module, [module.canvas]);
        } else {
            worker.postMessage(module);
        }
        workers.push(worker);
    }
}
main();