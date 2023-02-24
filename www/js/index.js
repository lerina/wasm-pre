import init, { Universe } from "../pkg/wasm_pre.js";

async function run() {
    const wasm = await init().catch(console.error);
    const memory = wasm.memory;

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();



//---------------------------- debug info
//console.log(player);
/*const player_pos = document.getElementById("player_pos");
const debug_info = () => {
     document.getElementById("bounds").innerHTML = "bounds: " + universe.width() 
                                                 + " X " + universe.height();
     player_pos.innerHTML = "player pos: x= " + universe.player_x() 
                            + " y= " + universe.player_y();   
};*/
//-------------------------------------------------------

const pre = document.getElementById("pre");

function keyDown(event) {
        //console.log("anim On: "+ `key=${event.key},code=${event.code}`);
        switch (event.keyCode) {
            case 32:
                //'spacebar Key pressed!';
                universe.player_shoot();
                break;
            case 37:
                //'Left Key pressed!';
                universe.move_player_left();
                console.log("<- x "+ universe.player_x());
                break;
            case 38:
                //'Up Key pressed!';
                universe.move_player_up();
                console.log("↑ y "+ universe.player_y());
//                event.detail.keyboardEvent.preventDefault();
                break;
            case 39:
                //'Right Key pressed!';
                universe.move_player_right();
                console.log("-> x "+ universe.player_x());
                break;
            case 40:
                //'Down Key pressed!';
                universe.move_player_down();
                //universe.play();
                console.log("↓ y "+ universe.player_y());
                // https://github.com/PolymerElements/iron-a11y-keys-behavior/issues/13
  //              event.detail.keyboardEvent.preventDefault();
                break;
        }
}
function keyUp(event) {
        //console.log("anim Off: "+ `key=${event.key},code=${event.code}`);
}

window.addEventListener('keydown', keyDown);
window.addEventListener('keyup', keyUp);

//---
const renderLoop = () => {
  pre.textContent = universe.render();
  //debug_info();

universe.tick();

  //requestAnimationFrame(renderLoop);
};

  requestAnimationFrame(renderLoop);
}//^--run

//-------------------
run();
