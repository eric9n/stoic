<script>
  import { invoke } from "@tauri-apps/api/core";
  import { Store } from "@tauri-apps/plugin-store";
  import { onMount } from "svelte";

  let errorMessage = "";
  let host = "";
  let schema = null;
  let port = null;
  let showToast = false;

  // 表单提交处理函数
  async function handleSubmit(event) {
    event.preventDefault();

    // const formData = new FormData(event.target);
    // const schema = formData.get("schema");
    // const port = formData.get("port");
    // const host = formData.get("host");
    try {
      const proxy = `${schema}://${host}:${port}`;
      await invoke("set_proxy", { proxy });
      console.log(proxy);
    } catch (error) {
      console.error("Error during invocation:", error);
      showToast = true;
    }
  }
  function handleFocus() {
    showToast = false;
  }

  onMount(async () => {
    const store = new Store("config");
    const proxy = await store.get("proxy");
    if (proxy) {
      const url = new URL(proxy);
      // 提取 schema, host 和 port
      schema = url.protocol.slice(0, -1); // 移除末尾的冒号
      host = url.hostname; // 获取主机名
      port = url.port;
    }
  });
</script>

<div class="w-full bg-base-100">
  <div class="overflow-x-auto">
    <div class="hero min-h-screen bg-base-100">
      <div class="hero-content flex-col lg:flex-row-reverse">
        <div class="card shrink-0 w-full shadow-2xl bg-base-100">
          {#if showToast}
            <div class="toast toast-top toast-center">
              <div role="alert" class="alert alert-error">
                <span>设置代理失败</span>
              </div>
            </div>
          {/if}
          <!-- svelte-ignore a11y-label-has-associated-control -->

          <form class="card-body w-96" on:submit|preventDefault={handleSubmit}>
            {#if errorMessage}
              <p class="error-message">{errorMessage}</p>
            {/if}
            <label class="label">
              <span class="label-text">代理设置</span>
            </label>
            <div class="form-control">
              <select
                name="schema"
                bind:value={schema}
                class="select select-bordered w-full max-w-xs"
                required
              >
                <option value="" selected>代理协议</option>
                <option value="http">HTTP</option>
                <option value="https">HTTPS</option>
                <option value="socks5">SOCKS5</option>
              </select>
            </div>
            <div class="form-control">
              <input
                bind:value={host}
                type="text"
                name="host"
                placeholder="代理服务器"
                class="input input-bordered"
                required
                spellcheck="false"
                on:focus={handleFocus}
              />
            </div>
            <div class="form-control">
              <input
                type="number"
                name="port"
                bind:value={port}
                placeholder="端口"
                class="input input-bordered"
                required
                spellcheck="false"
                on:focus={handleFocus}
              />
            </div>
            <!-- Updated container for the button -->
            <div class="mt-6 flex justify-center">
              <button class="btn btn-primary w-48">保存</button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</div>
