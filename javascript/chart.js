export class MyChart {
    constructor(id) {
        this.id = id
    }

    draw(config) {
        this.chart = new Chart(
            document.getElementById(this.id),
            config
        )
    }

    update(data) {
        this.chart.data = data
        this.chart.update()
    }
}