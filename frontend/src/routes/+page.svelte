<script lang="ts">
import Projects from './Projects.svelte'
  let name = "Coffinggx";
  let github = "https://github.com/coffinggx";
  let email = "mailto:coffinggx@gmail.com";
  let semiTitle = "Building and Learning Softwares";
</script>
<div class="flex w-full h-screen gap-10  flex-col text-center">
  <h1 class="text-3xl text-white font-mono mt-10"> {name.toUpperCase()} </h1>
  <a class="text-xl text-fuchsia-300 underline underline-offset-8"href={github} target="blank">GitHub</a>
  <a class="text-fuchsia-300 underline underline-offset-8" href={email} target="blank"> coffinggx [@] gmail [dot] com </a>
  <p class="text-xl text-lime-50"> -- {semiTitle} -- </p>
  <div class="text-white">
    <h1 class="text-2xl mt-10"> Projects </h1>
    <Projects/>
  </div>
</div>

