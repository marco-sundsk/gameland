let dice = document.getElementById('dice');
var outputDiv = document.getElementById('diceResult');
var rollEnd = false;
var t1;

function rollDice2() {
    let result = Math.floor(Math.random() * (6 - 1 + 1)) + 1;
    dice.dataset.side = result;
    dice.classList.toggle("reRoll");

    console.log(result);
	
    outputDiv.classList.remove("reveal");
    outputDiv.classList.add("hide");
		outputDiv.innerHTML = "You've got " + result;
		setTimeout(function(){ outputDiv.classList.add("reveal"); }, 1500);
}

function rollDice() {
	rollEnd = false;
	keepRoll();
	t1 = setInterval(keepRoll, 800);

	setTimeout(function()
	{ 
		let result = Math.floor(Math.random() * (6 - 1 + 1)) + 1;
		dice.dataset.side = result;
		rollEnd = true;
		// outputDiv.classList.add("reveal"); 
		// dice.classList.toggle("reRoll");
	 }, 5000);
}						

function keepRoll() {
	dice.classList.toggle("reRoll");
	if (rollEnd) {
		clearInterval(t1);
	}
}

dice.addEventListener("click", rollDice);