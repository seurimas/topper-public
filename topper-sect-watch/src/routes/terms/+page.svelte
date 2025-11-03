<script lang="ts">
    import Readme from "$lib/Readme.svelte";
</script>

<svelte:head>
	<title>Sect Watch - Terms</title>
</svelte:head>

<Readme title="Terms of Use">
    <h2>
        Seurimas' Sect Watch is provided "as is" without any warranties of any kind.
    </h2>

    <p>
        Data may persist or be removed at any time without notice. By using this service, you agree that the author is not responsible for any loss of data, service interruptions, or embarrassments that may occur from this tool.
    </p>

    <p>
        Please engage with this tool responsibly. Abuse of the service, including but not limited to uploading inappropriate content or attempting to compromise the system's security, may result in termination of access or legal action.
    </p>

    <p>
        Seurimas' Sect Watch is not affiliated with Aetolia or Iron Realms Entertainment. All trademarks and copyrights belong to their respective owners.
    </p>

    <p>
        By using Seurimas' Sect Watch, you agree to these terms. If you do not agree, please do not use the service.
    </p>

    <p>
        You may contact the author (Seurimas) on Discord for any questions or concerns regarding these terms, or to request data deletion.
    </p>

    <h2>Cookie and Privacy Policy</h2>

    <p>Cookies are used for security purposes, authentication, and session tracking.</p>

    <p>No personal data is shared with third parties beyond what is necessary for security, authentication, and as required by law.</p>
</Readme>

<style>
    @reference "tailwindcss";

	h2 {
		@apply text-2xl font-bold mt-6 mb-2;
	}

    p {
        @apply mb-4;
    }
</style>