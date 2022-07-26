export class BulmaCalendar {
    constructor(id) {
        this.id = id;
    }

    init(options, default_value) {
        this.calendars =
            bulmaCalendar.attach('.calendar_' + this.id, options);
        this.calendars.forEach(calendar => {
            if (default_value) {
                calendar.value(new Date(default_value));
            }
        });
    }

    on_select(callback) {
        this.element = document.querySelector('.calendar_' + this.id);
        if (this.element) {
            this.element.bulmaCalendar.on('save',
                datepicker => {
                    callback(datepicker.data.value())
                });
        }
    }

}