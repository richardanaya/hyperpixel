var HyperPixel = function(canvas){
    this.canvas = canvas;
    this.width = canvas.width;
    this.height = canvas.height;
    this.camera = new THREE.OrthographicCamera( 0, this.width, 0, this.height, 1, 2000 );
    this.camera.position.z = 2000;
    this.scene = new THREE.Scene();
    this.scene.fog = new THREE.Fog( 0x050505, 2000, 3500 );

    var particles = this.width*this.height;
    var geometry = new THREE.BufferGeometry();
    geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( new Array(particles*3), 3 ) );
    geometry.setAttribute( 'color', new THREE.Float32BufferAttribute( new Array(particles*3), 3 ) );
    var positions = geometry.attributes.position.array;
    this.colors = geometry.attributes.color.array;

    var i = 0;
    var w = this.width;
    var h = this.height;
    for ( var x = 0; x < w ; x++ ) {
        for ( var y = 0; y < h; y++ ){
            var i = (y*w+x)*3;
            positions[ i ]     = x;
            positions[ i + 1 ] = y+1;
            positions[ i + 2 ] = 10;
        }
    }

    var material = new THREE.PointsMaterial( { size: 1, vertexColors: THREE.VertexColors } );
    this.particleSystem = points = new THREE.Points( geometry, material );
    this.scene.add( this.particleSystem );
    this.renderer = new THREE.WebGLRenderer({canvas: this.canvas});
};

HyperPixel.prototype.render = function() {
    this.particleSystem.geometry.attributes.color.needsUpdate = true;
    this.renderer.render( this.scene, this.camera );
};
