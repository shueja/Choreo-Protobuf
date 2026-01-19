import './App.css'
import {Commands} from "./client"

function App() {
  
  return (
    <>
      <div className="card">
        <button onClick={() => {
          Commands.GetDefaultTrajectory().then(({trajectory})=>{
            console.log(trajectory);
            // mutate the trajectory
            trajectory.name = "HI MOM";
            return Commands.Generate({trajectory});}
          ).then((trajectory)=>console.log("result:", trajectory));
        }}>
          Generate!
        </button>
      </div>
    </>
  )
}

export default App
