<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref } from 'vue'

const props = withDefaults(
  defineProps<{
    startOnMount?: boolean
  }>(),
  {
    startOnMount: false,
  },
)

const canvasElement = ref<HTMLCanvasElement | null>(null)
const hasWebgl = ref(true)
const shouldRenderWebgl = ref(false)

let animationFrame = 0
let gl: WebGLRenderingContext | null = null
let program: WebGLProgram | null = null
let startedAt = 0

const vertexShaderSource = `
  attribute vec2 a_position;

  void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
  }
`

const fragmentShaderSource = `
  precision mediump float;

  uniform vec2 u_resolution;
  uniform float u_time;

  float grid(vec2 uv, float scale, float width) {
    vec2 cell = abs(fract(uv * scale) - 0.5);
    float line = min(cell.x, cell.y);
    return smoothstep(width, 0.0, line);
  }

  float scanline(vec2 uv) {
    return 0.5 + 0.5 * sin((uv.y + u_time * 0.22) * 900.0);
  }

  void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;
    vec2 centered = uv - 0.5;
    centered.x *= u_resolution.x / u_resolution.y;

    float vignette = smoothstep(0.92, 0.12, length(centered));
    float gridFine = grid(uv + vec2(u_time * 0.012, 0.0), 32.0, 0.014);
    float gridWide = grid(uv - vec2(0.0, u_time * 0.008), 8.0, 0.018);
    float cursor = smoothstep(0.02, 0.0, abs(fract(uv.x * 5.0 + u_time * 0.2) - 0.5));
    float terminalBand = smoothstep(0.18, 0.2, uv.y) * smoothstep(0.82, 0.8, uv.y);
    float glow = pow(max(0.0, 1.0 - length(centered * vec2(1.0, 1.45))), 3.0);

    vec3 black = vec3(0.039, 0.039, 0.039);
    vec3 card = vec3(0.102, 0.102, 0.102);
    vec3 yellow = vec3(0.980, 1.0, 0.412);
    vec3 gray = vec3(0.18);

    vec3 color = mix(black, card, terminalBand * 0.72);
    color += yellow * gridFine * 0.08 * vignette;
    color += yellow * gridWide * 0.11 * vignette;
    color += yellow * cursor * 0.05 * terminalBand;
    color += yellow * glow * 0.14;
    color += gray * scanline(uv) * 0.045;

    gl_FragColor = vec4(color, 1.0);
  }
`

function activateWebgl() {
  if (shouldRenderWebgl.value || !hasWebgl.value) {
    return
  }

  shouldRenderWebgl.value = true
  void nextTick(initializeWebgl)
}

function initializeWebgl() {
  const canvas = canvasElement.value

  if (!canvas) {
    return
  }

  gl = canvas.getContext('webgl', { antialias: false, alpha: false })

  if (!gl) {
    hasWebgl.value = false
    return
  }

  program = createProgram(gl, vertexShaderSource, fragmentShaderSource)

  if (!program) {
    hasWebgl.value = false
    return
  }

  const positionBuffer = gl.createBuffer()

  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
    gl.STATIC_DRAW,
  )

  const positionLocation = gl.getAttribLocation(program, 'a_position')

  gl.enableVertexAttribArray(positionLocation)
  gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0)
  startedAt = performance.now()
  render()
}

onBeforeUnmount(() => {
  cancelAnimationFrame(animationFrame)
})

onMounted(() => {
  if (props.startOnMount) {
    activateWebgl()
  }
})

function render() {
  if (!gl || !program || !canvasElement.value) {
    return
  }

  resizeCanvas(canvasElement.value)
  gl.viewport(0, 0, canvasElement.value.width, canvasElement.value.height)
  gl.useProgram(program)

  const resolutionLocation = gl.getUniformLocation(program, 'u_resolution')
  const timeLocation = gl.getUniformLocation(program, 'u_time')

  gl.uniform2f(resolutionLocation, canvasElement.value.width, canvasElement.value.height)
  gl.uniform1f(timeLocation, (performance.now() - startedAt) / 1000)
  gl.drawArrays(gl.TRIANGLES, 0, 6)
  animationFrame = requestAnimationFrame(render)
}

function resizeCanvas(canvas: HTMLCanvasElement) {
  const pixelRatio = window.devicePixelRatio || 1
  const width = Math.max(1, Math.floor(canvas.clientWidth * pixelRatio))
  const height = Math.max(1, Math.floor(canvas.clientHeight * pixelRatio))

  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width
    canvas.height = height
  }
}

function createProgram(
  targetGl: WebGLRenderingContext,
  vertexSource: string,
  fragmentSource: string,
) {
  const vertexShader = createShader(targetGl, targetGl.VERTEX_SHADER, vertexSource)
  const fragmentShader = createShader(targetGl, targetGl.FRAGMENT_SHADER, fragmentSource)

  if (!vertexShader || !fragmentShader) {
    return null
  }

  const nextProgram = targetGl.createProgram()

  if (!nextProgram) {
    return null
  }

  targetGl.attachShader(nextProgram, vertexShader)
  targetGl.attachShader(nextProgram, fragmentShader)
  targetGl.linkProgram(nextProgram)

  if (!targetGl.getProgramParameter(nextProgram, targetGl.LINK_STATUS)) {
    return null
  }

  return nextProgram
}

function createShader(targetGl: WebGLRenderingContext, type: number, source: string) {
  const shader = targetGl.createShader(type)

  if (!shader) {
    return null
  }

  targetGl.shaderSource(shader, source)
  targetGl.compileShader(shader)

  if (!targetGl.getShaderParameter(shader, targetGl.COMPILE_STATUS)) {
    return null
  }

  return shader
}
</script>

<template>
  <div
    class="relative min-h-[24rem] overflow-hidden rounded-xl border border-[#2a2a2a] bg-[#1a1a1a]"
    @click="activateWebgl"
    @focusin="activateWebgl"
    @pointerenter="activateWebgl"
  >
    <div
      class="absolute inset-0 bg-[radial-gradient(circle_at_50%_28%,rgba(250,255,105,0.16),transparent_34%),linear-gradient(90deg,rgba(250,255,105,0.08)_1px,transparent_1px),linear-gradient(0deg,rgba(250,255,105,0.06)_1px,transparent_1px)] bg-[length:auto,2rem_2rem,2rem_2rem]"
      aria-hidden="true"
    />

    <canvas
      v-if="shouldRenderWebgl && hasWebgl"
      ref="canvasElement"
      class="absolute inset-0 h-full w-full"
      aria-hidden="true"
      data-testid="webgl-hero-canvas"
    />

    <div class="relative grid h-full min-h-[24rem] content-between p-5">
      <div class="flex items-center justify-between border-b border-[#2a2a2a] pb-3 font-mono text-xs text-[#888888]">
        <span>query-in://local-engine</span>
        <span class="text-[#faff69]">WASM READY</span>
      </div>

      <pre class="overflow-hidden font-mono text-sm leading-7 text-white"><code><span class="text-[#888888]">01</span> select region, revenue
<span class="text-[#888888]">02</span> from uploaded_csv
<span class="text-[#888888]">03</span> where browser_only = true
<span class="text-[#888888]">04</span> limit 100;</code></pre>

      <div class="grid gap-3 sm:grid-cols-3">
        <div class="border-l border-[#faff69] pl-3">
          <p class="font-mono text-2xl font-bold text-[#faff69]">0</p>
          <p class="text-xs text-[#888888]">server uploads</p>
        </div>
        <div class="border-l border-[#faff69] pl-3">
          <p class="font-mono text-2xl font-bold text-[#faff69]">WASM</p>
          <p class="text-xs text-[#888888]">query runtime</p>
        </div>
        <div class="border-l border-[#faff69] pl-3">
          <p class="font-mono text-2xl font-bold text-[#faff69]">SQL</p>
          <p class="text-xs text-[#888888]">local analysis</p>
        </div>
      </div>
    </div>
  </div>
</template>
