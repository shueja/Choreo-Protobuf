import { useState } from 'react'
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
async function getTrajectory(){
  const response = await client.GetDefaultTrajectory({});
  return response.trajectory;
}
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
      <div className="card">
        <button onClick={() => {
          setCount((count) => count + 1);
          echoSwerveSample();
          getTrajectory().then((trajectory)=>{
            console.log(trajectory);
            // mutate the trajectory
            trajectory!.name = "HI MOM";
            return client.Generate({trajectory});}
          ).then((trajectory)=>console.log("result:", trajectory));
        }}>
          Generate!
        </button>
      </div>
    </>
  )
}

export default App
