var HyperPixel = function(canvas){
    this.canvas = canvas;
    this.width = canvas.width;
    this.height = canvas.height;
    this.camera = new THREE.OrthographicCamera( 0, this.width, 0, this.height, - 2000, 2000 );
    this.camera.position.z = 2000;

    this.scene = new THREE.Scene();
    this.scene.fog = new THREE.Fog( 0x050505, 2000, 3500 );

    this.renderer = null;
    this.particleSystem = null;

    var particles = this.width*this.height;

    var geometry = new THREE.BufferGeometry();
    geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( new Array(particles*3), 3 ) );
		geometry.setAttribute( 'color', new THREE.Float32BufferAttribute( new Array(particles*3), 3 ) );

    var positions = geometry.attributes.position.array;
    this.colors = geometry.attributes.color.array;

    var i = 0;
    var w = this.width;
    var h = this.height;
    var halfPixel = this.pixelSize/2;
    for ( var x = 0; x < w; x++ ) {
        for ( var y = 0; y < h; y++ ){
            var i = (y*h+x)*3;
            positions[ i ]     = x+.5;
            positions[ i + 1 ] = y+.5;
            positions[ i + 2 ] = 0;
        }
    }

    var material = new THREE.PointsMaterial( { size: 15, vertexColors: THREE.VertexColors } );
    this.particleSystem = points = new THREE.Points( geometry, material );
    this.scene.add( this.particleSystem );

    this.renderer = new THREE.WebGLRenderer( {canvas: this.canvas, antialias: false, clearColor: 0x333333, clearAlpha: 1, alpha: false } );
    this.renderer.setSize( this.width, this.height );
    this.renderer.setClearColor( this.scene.fog.color, 1 );
};

HyperPixel.prototype.render = function() {
    this.particleSystem.geometry.attributes.color.needsUpdate = true;
    this.renderer.render( this.scene, this.camera );
};
