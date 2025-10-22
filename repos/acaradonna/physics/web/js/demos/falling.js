import { createEngine } from '../engine.js';

const cfg = { running: true };

function rand(min,max){ return Math.random()*(max-min)+min }

function draw(ctx, bodies){
  ctx.clearRect(0,0,ctx.canvas.width,ctx.canvas.height);
  ctx.save();
  ctx.fillStyle = '#66d9ef';
  for (const b of bodies){
    const x = (b.position.x*10)+ctx.canvas.width*0.5;
    const y = ctx.canvas.height*0.2 + (b.position.y*-10);
    ctx.beginPath(); ctx.arc(x,y,2,0,Math.PI*2); ctx.fill();
  }
  ctx.restore();
}

async function main(){
  const { World } = await createEngine();
  const canvas = document.getElementById('falling-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const world = new World();

  const gravitySlider = document.getElementById('falling-gravity');
  const countSlider = document.getElementById('falling-count');
  const resetBtn = document.getElementById('falling-reset');
  const toggleBtn = document.getElementById('falling-toggle');

  function reset(){
    world.bodies = [];
    for (let i=0;i<Number(countSlider.value);i++){
      world.createRigidBody({ position:{x:rand(-5,5), y:rand(2,20), z:0}, velocity:{x:0,y:0,z:0}, mass:1 });
    }
  }

  reset();

  gravitySlider.addEventListener('input', ()=>{
    world.setGravity({x:0,y:Number(gravitySlider.value),z:0});
  });
  resetBtn.addEventListener('click', reset);
  toggleBtn.addEventListener('click', ()=>{ cfg.running = !cfg.running; toggleBtn.textContent = cfg.running ? 'Pause' : 'Resume'; });
  countSlider.addEventListener('change', reset);

  let last = performance.now();
  function loop(t){
    const dt = Math.min(1/60, (t-last)/1000); last = t;
    if (cfg.running){
      // fixed 120 Hz internal for smoother motion
      const sub = 2; const subdt = dt/sub;
      for (let i=0;i<sub;i++) world.step(subdt);
    }
    draw(ctx, world.bodies);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
}

main();
