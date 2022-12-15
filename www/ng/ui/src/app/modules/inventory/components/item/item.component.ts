import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-item',
  templateUrl: './item.component.html',
  styleUrls: ['./item.component.css']
})
export class ItemComponent implements OnInit {

  formItem = new FormGroup({
    name: new FormControl('', []),
    description: new FormControl('', []),
    sku: new FormControl('', []),
    upc: new FormControl('', [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Item");
  }

  ngOnInit(): void {
  }

  submit() {

  }
}
