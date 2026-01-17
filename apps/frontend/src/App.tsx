import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import {service} from "@choreo/proto"
import './App.css'
import { grpc } from '@improbable-eng/grpc-web';

const rpc = new service.GrpcWebImpl('http://localhost:50051', {
  // Only necessary for tests running on node. Remove the
  // transport config when actually using in the browser.
  debug: false,
  metadata: new grpc.Metadata({ SomeHeader: 'bar' }),
});
const client = new service.ChoreoServiceClientImpl(rpc);
function App() {
  const [count, setCount] = useState(0)
  async function echoSwerveSample() {
    const {sample} = await client.EchoSwerveSample({sample: {
    t: count,
    x: 0,
    y: 0,
    heading: 0,
    vx: 0,
    vy: 0,
    omega: 0,
    ax: 0,
    ay: 0,
    alpha: 0,
    fl: undefined,
    fr: undefined,
    bl: undefined,
    br: undefined
} });
  console.log(sample);
  }
  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => {
          setCount((count) => count + 1);
          echoSwerveSample();
        }}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
