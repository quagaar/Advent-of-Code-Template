create day:
    cargo generate --name day{% raw %}{{day}}{% endraw %} --define aocyear={{aocyear}} --define aocday={% raw %}{{trim_start_matches(day, "0")}}{% endraw %} --vcs None --git https://github.com/quagaar/Advent-of-Code-Template.git day
