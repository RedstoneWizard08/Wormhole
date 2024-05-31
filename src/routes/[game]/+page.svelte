<script lang="ts">
import { goto } from "$app/navigation";
import { page } from "$app/stores";

const gameId = Number.parseInt($page.params.game);

goto(`/${gameId}/instances`);
</script>
