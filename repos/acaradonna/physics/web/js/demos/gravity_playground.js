import { createEngine } from '../engine.js';

const cfg = { running: true };
function rand(min,max){ return Math.random()*(max-min)+min }

function draw(ctx, bodies){
  ctx.clearRect(0,0,ctx.canvas.width,ctx.canvas.height);
  ctx.save();
  ctx.fillStyle = '#c678dd';
  for (const b of bodies){
    const x = (b.position.x*10)+ctx.canvas.width*0.5;
    const y = ctx.canvas.height*0.5 + (b.position.y*-10);
    ctx.fillRect(x-1,y-1,2,2);
  }
  ctx.restore();
}

async function main(){
  const { World } = await createEngine();
  const canvas = document.getElementById('gp-canvas');
  if (!canvas) return;
  const ctx = canvas.getContext('2d');
  const world = new World();

  const gx = document.getElementById('gx');
  const gy = document.getElementById('gy');
  const gz = document.getElementById('gz');
  const spawnBtn = document.getElementById('gp-spawn');
  const clearBtn = document.getElementById('gp-clear');
  const toggleBtn = document.getElementById('gp-toggle');

  function spawn(n){
    for (let i=0;i<n;i++) world.createRigidBody({ position:{x:rand(-5,5), y:rand(-1,5), z:rand(-5,5)}, velocity:{x:0,y:0,z:0}, mass:1 });
  }

  function updateG(){ world.setGravity({x:Number(gx.value), y:Number(gy.value), z:Number(gz.value)}); }
  updateG();

  spawnBtn.addEventListener('click', ()=>spawn(100));
  clearBtn.addEventListener('click', ()=>{ world.bodies = []; });
  toggleBtn.addEventListener('click', ()=>{ cfg.running = !cfg.running; toggleBtn.textContent = cfg.running ? 'Pause' : 'Resume'; });
  gx.addEventListener('input', updateG); gy.addEventListener('input', updateG); gz.addEventListener('input', updateG);

  let last = performance.now();
  function loop(t){
    const dt = Math.min(1/60, (t-last)/1000); last = t;
    if (cfg.running){ const sub = 2; const subdt = dt/sub; for (let i=0;i<sub;i++) world.step(subdt); }
    draw(ctx, world.bodies);
    requestAnimationFrame(loop);
  }
  requestAnimationFrame(loop);
}

main();
