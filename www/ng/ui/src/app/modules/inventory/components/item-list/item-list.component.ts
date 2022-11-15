import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-item-list',
  templateUrl: './item-list.component.html',
  styleUrls: ['./item-list.component.css']
})
export class ItemListComponent implements OnInit {

  formItems = new FormGroup({
    filter: new FormControl('', []),
    items: new FormControl(10, []),
    page: new FormControl(1, [])
  });

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Items");
  }

  ngOnInit(): void {
  }

  submit() {
    console.log("ItemListComponent::submit()");
  }
}
