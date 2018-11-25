<template>
  <div id="app">
    <input id="button" type="button" value="実行" v-on:click="click" /><br>
    <textarea id="code_input_area" v-model="code"></textarea>
    <div id="result">
      <h2>Execution</h2>
      <div v-html="result"></div>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import axios from "axios";

@Component({
  components: {},
  props: {
    code: String
  }
})
export default class App extends Vue {
  private code = "";
  private result = "result show area";
  async click() {
    this.result = (await axios.post("/compile", {
      code: this.code
    })).data;
    this.result = this.result.replace(/\n/g, "<br>");
  }
}
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 30px;
  margin-left: 10px;
  margin-right: 10px;
}

#code_input_area {
  width: 100%;
  height: 300px;
  font-size: 24px;
}

#button {
  display: block;
  font-size: 32px;
  width: 150px;
  height: 50px;
}

#result {
  background-color: white;
}
</style>
