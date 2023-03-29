<script lang="ts">
    export let bytes: number[];
    $: chars = bytes
        .map((byte) => {
            if (byte === 0x0a) {
                return "\u2022";
            }

            return String.fromCharCode(byte);
        })
        .join("");
</script>

<div>
    {#each bytes as byte}
        <span id="byte">
            {byte.toString(16).padStart(2, "0").toUpperCase()}
        </span>
    {/each}
    <span class="noselect" data-string={chars} />
</div>

<style>
    span {
        font-family: "Source Code Pro", "Courier New", Consolas, Menlo,
            "PT Mono", "Liberation Mono", monospace;
    }

    span#byte {
        width: auto;
        padding: 1px;
        padding-inline-start: 4px;
        padding-inline-end: 5px;
    }

    span#byte:hover {
        background-color: #444;
    }

    span.noselect {
        user-select: none;
        -webkit-user-select: none;
    }

    span.noselect::before {
        content: attr(data-string);
    }
</style>
