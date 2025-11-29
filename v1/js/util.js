var Util = new Object();

(function(util) {

	this.extend = function(x,y) {
		if (y) {
			for ( var _y in y ) {
				x[_y] = y[_y];
			}
		}
	};

})(Util);