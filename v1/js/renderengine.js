
var RenderEngine = new Object();

(function(re) {

	var Engine = function() {
		var _canvas;
		var _context;
		var _gameBoard;
		var _squareSize;
		var _borderSize;
		var _startFlag;
		
		this.set_gameBoard = function(gameBoard) { _gameBoard = gameBoard; }
		
		this.prepareCanvas = function(canvasId, squareSize, borderSize) {
			if (!_gameBoard)
				throw "Game Board not set";
			
			_canvas = document.getElementById(canvasId);
			if (!_canvas)
				return false;
				
			_squareSize = squareSize;
			_borderSize = borderSize;
			_canvas.width = _squareSize * _gameBoard.get_width() + _borderSize * 2;
			_canvas.height = _squareSize * _gameBoard.get_height() + _borderSize * 2;
			
			_context = _canvas.getContext('2d');
			if (!_context)
				return false;
			
			return true;
		};
		
		this.render = function() {
			
			clearScreen();
			
			if (_gameBoard.get_IsStarted()) {
				renderGameBoard();
			} else {
				renderStartScreen();
			}
			
			renderBorder();
		};
		
		var clearScreen = function () {
			_context.fillStyle = "#FFFFFF";
			_context.fillRect(_borderSize,_borderSize,_canvas.width - _borderSize * 2,_canvas.height - _borderSize * 2);
		};
		
		var renderBorder = function() {
			_context.font = "16px Arial";
			_context.strokeStyle = "#696969";
			_context.fillStyle = "#696969";
			_context.lineWidth = _borderSize;
			_context.strokeRect(_borderSize / 2,_borderSize / 2,_canvas.width - _borderSize,_canvas.height - _borderSize);
			_context.lineWidth = 1;
			_context.strokeStyle = "#000000";
			_context.strokeRect(1,1,_canvas.width - 2, _canvas.height - 2);
			
			if (_gameBoard.get_IsStarted()) {
				var x = _borderSize + 2;
				var y = _borderSize + 2;
				_context.fillText("Level: " + _gameBoard.get_level(), _borderSize + 5, 30);
				_context.fillText("Score: " + _gameBoard.get_score(), _borderSize + 5, 50);
				
				renderShape(_gameBoard.get_nextShape(), 0, 0, 15, _canvas.width - 50, 30);
			}
		};
		
		var renderStartScreen = function() {
			if (_startFlag) {
				_context.font = "30px Arial";
				_context.strokeStyle = "#00FF00";
				var info = _context.measureText("Press Space to Start!");
				var x = (_canvas.width - info.width) / 2;
				var y = (_canvas.height) / 2;
				_context.strokeText("Press Space to Start!", x, y);
				_startFlag = false;
			} else {
				_startFlag = true;
			}
		};
		
		var renderGameBoard = function () {
			var J = _gameBoard.get_height();
			var I = _gameBoard.get_width();
			var G = _gameBoard.get_grid();
			var c;
			for (var j = 0; j < J; ++j) {
				for (var i = 0; i < I; ++i) {
					c = G[j][i].get_color();
					renderSquare(i * _squareSize + _borderSize,j * _squareSize + _borderSize,_squareSize, c);
				}
			}
			
			var shape = _gameBoard.get_currentShape();
			var x = shape.get_X();
			var y = shape.get_Y();
			renderShape(shape, x, y, _squareSize, _borderSize, _borderSize);
		};
		
		var renderSquare = function (x,y,s,c) {
			if (c > 0) {
				switch ( c ) {
					case 1:
					_context.strokeStyle = "#FF0000";
					_context.fillStyle = "#FF0000";
					break;
					case 2:
					_context.strokeStyle = "#00FF00";
					_context.fillStyle = "#00FF00";
					break;
					case 3:
					_context.strokeStyle = "#0000FF";
					_context.fillStyle = "#0000FF";
					break;
					case 4:
					_context.strokeStyle = "#FF00FF";
					_context.fillStyle = "#FF00FF";
					break;
					case 5:
					_context.strokeStyle = "#00FFFF";
					_context.fillStyle = "#00FFFF";
					break;
					case 6:
					_context.strokeStyle = "#FFFF00";
					_context.fillStyle = "#FFFF00";
					break;
					default:
					_context.strokeStyle = "#000000";
					_context.fillStyle = "#000000";
					break;
				}
				_context.strokeRect(x,y,s,s);
				_context.fillRect(x+2,y+2,s-4,s-4);
			}
		};
		
		var renderShape = function (shape, x, y, sz, offsetX, offsetY) {
			var squares = shape.get_squares();
			for (var index in squares) {
				var px = (x + squares[index].x) * sz + offsetX;
				var py = (y + squares[index].y) * sz + offsetY;
				renderSquare(px, py, sz, shape.get_color());
			}
		};
	};

	re.create = function() {
		return new Engine();
	};

})(RenderEngine);