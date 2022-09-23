<template>
	<div class="container">
		<div class="background"
			:style="{'background-image': 'url(' + (processedGames.length > 0 ? processedGames[selectedGame].background : ``) + ')'}">
		</div>
		<Header />
		<div class="app-container">
			<Loader v-if="processedGames.length != games.length" />
			<div class="games-container" v-else>
				<Game :id="processedGames.game_id" v-for="(game, index) in processedGames" :key="index" :game="game"
					:class="{'selected' : processedGames[selectedGame].game_id == game.game_id}" />
			</div>
			<div class="menu-container" v-if="showingMenu">
				<div class="menu-background"></div>
				<div class="menu-box">
					<div class="menu-options-container">
						<MenuOption v-for="(option, index) in options" :key="index" :optionLabel="option.label"
							:class="{'selected' : options[selectedOption] == option}" />
					</div>
				</div>
			</div>
		</div>
		<Footer :showingMenu="showingMenu" />
	</div>
</template>

<script>
import Header from "./components/Header.vue";
import Footer from "./components/Footer.vue";
import Game from "./components/Game.vue";
import Loader from "./components/Loader.vue"
import axios from "axios"
import MenuOption from "./components/MenuOption.vue";

import { invoke } from '@tauri-apps/api'

export default {
	name: "App",
	data() {
		return {
			showingMenu: false,
			selectedGame: 0,
			games: [
				{
					"game_id": 413150,
				},
				{
					"game_id": 105600,
				},
				{
					"game_id": 1621690,
				},
				{
					"game_id": 641990,
				}
			],
			selectedOption: 0,
			options: [
				{
					"label": "Riavvia il dispositivo",
					"action": this.restart
				},
				{
					"label": "Spegni dispositivo",
					"action": this.turnOff
				},

			],
			processedGames: [],
			allowUp: true,
			allowDown: true,
			allowLeft: true,
			allowRight: true,
			allowA: true,
			allowB: true,
			allowX: true
		}
	},
	components: {
		Header,
		Footer,
		Game,
		Loader,
		MenuOption
	},
	methods: {
		setupGamepad() {
			let gp = [];
			
			let haveEvents = "ongamepadconnected" in window;
			window.addEventListener("gamepadconnected", connectGamepad);
			window.addEventListener("gamepaddisconnected", disconnectGamepad);
			let self = this
			console.log("AAA")
			function disconnectGamepad() {
				gp = [];
			}

			function connectGamepad(e) {
				console.log("trying to connect");
				gp[0] = e.gamepad;
				requestAnimationFrame(updateStatus);
			}

			function updateStatus() {
				if (!haveEvents) {
					scangamepads();
				}

				for (let i = 0; i < gp[0].buttons.length; i++) {
					//up
					if (gp[0].buttons[12].pressed) {
						if (self.allowUp) {
							self.gamepadUpPressed()
						}
					}

					//down
					if (gp[0].buttons[13].pressed) {
						if (self.allowDown) {
							self.gamepadDownPressed()
						}
					}

					//left
					if (gp[0].buttons[14].pressed) {
						if (self.allowLeft) {
							self.gamepadLeftPressed()
						}
					}

					//up
					if (gp[0].buttons[15].pressed) {
						if (self.allowRight) {
							self.gamepadRightPressed()
						}
					}

					//A
					if (gp[0].buttons[0].pressed) {
						if (self.allowA) {
							self.gamepadAPressed()
						}
					}

					//B
					if (gp[0].buttons[1].pressed) {
						if (self.allowB) {
							self.gamepadBPressed()
						}
					}

					//X
					if (gp[0].buttons[2].pressed) {
						if (self.allowX) {
							self.gamepadXPressed()
						}
					}
				}

				for (let j = 0; j < gp[0].axes.length; j++) {
					//console.log(gp[0].axes[3]);
					if (gp[0].axes[1] < -0.8 || gp[0].axes[3] < -0.8) {
						if (self.allowUp) {
							self.gamepadUpPressed()
						}
					}
					if (gp[0].axes[1] > 0.8 || gp[0].axes[3] > 0.8) {
						if (self.allowDown) {
							self.gamepadDownPressed()
						}
					}
					if (gp[0].axes[0] < -0.8 || gp[0].axes[2] < -0.8) {
						if (self.allowLeft) {
							self.gamepadLeftPressed()
						}
					}
					if (gp[0].axes[0] > 0.8 || gp[0].axes[2] > 0.8) {
						if (self.allowRight) {
							self.gamepadRightPressed()
						}
					}
				}

				requestAnimationFrame(updateStatus);
			}

			function scangamepads() {
				var gamepads = navigator.getGamepads
					? navigator.getGamepads()
					: navigator.webkitGetGamepads
						? navigator.webkitGetGamepads()
						: [];
				for (var i = 0; i < gamepads.length; i++) {
					if (gamepads[i]) {
						if (gamepads[i].index in gp) {
							gp[gamepads[i].index] = gamepads[i];
						} else {
							addgamepad(gamepads[i]);
						}
					}
				}
			}

			if (!haveEvents) {
				setInterval(scangamepads, 500);
			}


		},
		onGameSelected(id) {
			const el = document.getElementById(id)
			if (el) {
				el.scrollIntoView({ behavior: "smooth", block: "end", inline: "center" })
			}
		},
		setupGames() {
			this.games.forEach((game) => {
				axios.get(`https://api.codetabs.com/v1/proxy/?quest=https://store.steampowered.com/api/appdetails?appids=${game.game_id}`, {
					mode: 'no-cors'
				})
					.then(response => {
						let gamedata = response.data[`${game.game_id}`].data


						this.processedGames.push({
							"game_id": game.game_id,
							"name": gamedata.name,
							"background": gamedata.background,
							"cover": `https://cdn.cloudflare.steamstatic.com/steam/apps/${game.game_id}/library_600x900.jpg`
						})


					})
			})


		},
		turnOff() {
			console.log("turn off")
		},
		restart() {
			console.log("restart")
		},
		gamepadUpPressed() {
			if (this.showingMenu) {
				this.selectedOption = this.selectedOption >= 1 ? (this.selectedOption - 1) : (this.options.length - 1)
			}
			this.allowUp = false
			setTimeout(() => {
				this.allowUp = true;
			}, 300);
		},

		gamepadDownPressed() {
			if (this.showingMenu) {
				this.selectedOption = this.selectedOption < this.options.length - 1 ? this.selectedOption + 1 : 0
			}
			this.allowDown = false
			setTimeout(() => {
				this.allowDown = true;
			}, 300);
		},

		gamepadLeftPressed() {
			if (!this.showingMenu) {
				this.selectedGame = this.selectedGame >= 1 ? (this.selectedGame - 1) : (this.games.length - 1)
				this.onGameSelected(this.games[this.selectedGame].game_id)
			}

			this.allowLeft = false
			setTimeout(() => {
				this.allowLeft = true;
			}, 300);
		},

		gamepadRightPressed() {
			if (!this.showingMenu) {
				this.selectedGame = this.selectedGame < this.games.length - 1 ? this.selectedGame + 1 : 0
				this.onGameSelected(this.games[this.selectedGame].game_id)
			}

			this.allowRight = false
			setTimeout(() => {
				this.allowRight = true;
			}, 300);
		},

		gamepadAPressed() {
			if (this.showingMenu) {
				let option = this.options[this.selectedOption]
				option.action()
			} else {
				console.log("seleziono gioco " + this.processedGames[this.selectedGame].name)
				invoke('open_game', { id: this.processedGames[this.selectedGame].game_id })
					.then((response) => console.log(response))
			}
			this.allowA = false
			setTimeout(() => {
				this.allowA = true;
			}, 300);
		},

		gamepadBPressed() {
			if (this.showingMenu) {
				this.showingMenu = false
			}

			this.allowB = false
			setTimeout(() => {
				this.allowB = true;
			}, 300);
		},

		gamepadXPressed() {
			if (!this.showingMenu) {
				this.showingMenu = true
			}

			this.allowX = false
			setTimeout(() => {
				this.allowX = true;
			}, 300);
		}
	},
	mounted() {
		this.setupGamepad()
		this.setupGames()

		document.onkeydown = checkKey;

		var self = this

		function checkKey(e) {

			e = e || window.event;

			if (e.keyCode == '38') {
				// up arrow
			}
			else if (e.keyCode == '40') {
				// down arrow
			}
			else if (e.keyCode == '37') {
				// left arrow
				self.gamepadLeftPressed()
			}
			else if (e.keyCode == '39') {
				// right arrow
				self.gamepadRightPressed()
			}
			else if (e.keyCode == '13') {
				// enter
				self.gamepadAPressed()
			}

		}
	}
}
</script>

<style scoped>
.container {
	width: 100%;
	height: 100%;
	/*
		background: linear-gradient(-35deg, #0C8749, #0C4987, #710977, #770920, #095777, #B11FAE);
		background-size: 400% 400%;
		animation: gradient 10s ease infinite;
		*/
	background: transparent;
	display: flex;
	flex-direction: column;
}

.background {
	position: absolute;
	width: 100vw;
	height: 100vh;
	left: 0;
	top: 0;
	opacity: 1;
	background-size: cover;
	background-repeat: no-repeat;
	background-position: center;
	z-index: -1;
}

.app-container {
	flex-grow: 1;
	position: relative;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	gap: 35px;
}

@keyframes gradient {
	0% {
		background-position: 0% 50%;
	}

	50% {
		background-position: 100% 50%;
	}

	100% {
		background-position: 0% 50%;
	}
}

.games-container {
	width: 100%;
	height: 52vh;
	overflow-x: scroll;
	white-space: nowrap;
	overflow-y: hidden;
	padding: 3vh 2vw;
	box-sizing: border-box;
}

.games-container::-webkit-scrollbar {
	display: none;
	height: 0;
}

.games-container {
	-ms-overflow-style: none;
	/* IE and Edge */
	scrollbar-width: none;
	/* Firefox */
}

.game-info {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	gap: 2vh;
}

.game-logo {
	width: auto;
	height: 15vh;
}

.menu-container {
	position: absolute;
	width: 100%;
	height: 100%;
	left: 0;
	top: 0;
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
}

.menu-background {
	position: absolute;
	background: #091923;
	width: 100%;
	height: 100%;
	opacity: 0.9;
	left: 0;
	top: 0;
}

.menu-box {
	height: 100%;
	width: 40vw;
	padding: 2vw;
	box-sizing: border-box;
	border-radius: 0.2vw;
	z-index: 1;
	display: flex;
	flex-direction: column;
	justify-content: center;
}

.menu-options-container {
	display: flex;
	flex-direction: column;
	justify-content: center;
	align-items: center;
	gap: 0.75vh;
}
</style>
