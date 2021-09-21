import { expose, proxy } from '//cdn.jsdelivr.net/npm/comlink@4.3.1/dist/esm/comlink.js'
import init, { initThreadPool, spawnTask } from './rust_async.js'
async function main() {
    await init()
    await initThreadPool(navigator.hardwareConcurrency)
    return proxy({
        spawnTask
    })
}

expose({
    module: main()
})