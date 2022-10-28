<script lang="ts">
export default {
  data: () => ({
    width: document.documentElement.clientWidth,
    height: document.documentElement.clientHeight,
    msg: "lol"
  }),
  mounted() {
    window.addEventListener('resize', this.getDimensions);
  },
  unmounted() {
    window.removeEventListener('resize', this.getDimensions);
  },
  methods: {
    getDimensions() {
      this.width = document.documentElement.clientWidth;
      this.height = document.documentElement.clientHeight;
      this.msg = "idiot";
    },
    drawCanvas() {
      var c: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("myCanvas")!;
      var ctx = c.getContext("2d")!;
      ctx.moveTo(0, 0);
      ctx.lineTo(200, 100);
      ctx.strokeStyle = '#ff000';
      ctx.stroke();
    }
  }
}
</script>

<template>
  <div class="card">
    <canvas id="myCanvas" vbind:height="height" vbind:width="width"></canvas>
  </div>
  <button @click="drawCanvas">
    {{msg}}
  </button>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
