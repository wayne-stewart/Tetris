
var Application = new Object();

(function(app) {

	var _renderEngine;
	var _gameBoard;
	var _gameLoopTimer;
	var _gameLoopInterval = 300;
	
	app.start = function(canvasId) {
	
		document.onkeydown = function (e) {
			
			var render = false;
			
			switch (e.which) {
				case 32: // space
					if (!_gameBoard.get_IsStarted()) {
						_gameBoard.start();
						_gameBoard.setup();
					} else if (_gameBoard.get_IsPaused()) {
						_gameBoard.start();
					} else
						render = _gameBoard.tryRotate();
				break;
				case 37: // left
				render = _gameBoard.tryShiftLeft();
				break;
				case 39: // right
				render = _gameBoard.tryShiftRight();
				break;
				case 40: // down
				render = _gameBoard.tryShiftDown();
				break;
				case 80: // p
				_gameBoard.pause();
				break;
			}
			
			if (render) {
				_renderEngine.render();
			}
		};
	
		_renderEngine = RenderEngine.create();
		
		_gameBoard = GameBoard.create(15,25);
		
		_gameBoard.levelIncrease = onLevelIncrease;
			
		_renderEngine.set_gameBoard(_gameBoard);
		
		_renderEngine.prepareCanvas(canvasId, 25, 5);
		
		_gameBoard.setup();
		
		startGameLoop(_gameLoopInterval);
	};
	
	var startGameLoop = function(intervalTime) {
		_gameLoopTimer = setInterval(function() {
			if (_gameBoard.get_IsPlaying()) {
				_gameBoard.tryShiftDown();
			} 
			
			_renderEngine.render();
		}, intervalTime);
	};
	
	var onLevelIncrease = function() {
		clearInterval(_gameLoopTimer);
		_gameLoopInterval -= 25;
		if (_gameLoopInterval < 0)
			_gameLoopInterval = 0;
		startGameLoop(_gameLoopInterval);
	};

})(Application);