export class MyChart {
    constructor() {
        let labels = [
            'January',
            'February',
            'March',
            'April',
            'May',
            'June',
        ];
        
        let data = {
            labels: labels,
            datasets: [{
                label: 'My First dataset',
                backgroundColor: 'rgb(255, 99, 132)',
                borderColor: 'rgb(255, 99, 132)',
                data: [0, 10, 5, 2, 20, 30, 45],
            }]
        };

        this.config = {
            type: 'line',
            data: data,
            options: {
                responsive: false
            }
        };
    }

    draw() {
        new Chart(
            document.getElementById('myChart'),
            this.config
        )
    }
}