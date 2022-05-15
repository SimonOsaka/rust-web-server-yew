export class MyChart {
    constructor() {
    }

    draw(config) {
        this.chart = new Chart(
            document.getElementById('mychart'),
            config
        )
    }

    update(data) {
        this.chart.data = data
        this.chart.update()
    }
}