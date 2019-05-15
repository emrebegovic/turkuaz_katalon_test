package com.turkuaz.test.pkg

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.google.common.collect.ArrayTable.Column
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

import org.apache.poi.ss.usermodel.Cell;
import org.apache.poi.ss.usermodel.Row;
import org.apache.poi.xssf.usermodel.XSSFSheet;
import org.apache.poi.xssf.usermodel.XSSFWorkbook;



public class getTransactions {

	@Keyword
	public String getMSISDN() {


		String id="1"
		def z = [1, 2, 3, 4, 5, 6, 7, 8, 9]
		Random rnd = new Random()

		for (int i=0; i < 8; i++) {
			id= id +rnd.nextInt(z.size)
		};


		System.out.println(">>>> MSISDN _ID = "  +id);
		return id
	}


	@Keyword
	public String getIMSI() {


		String id="1"
		def z = [1, 2, 3, 4, 5, 6, 7, 8, 9]
		Random rnd = new Random()

		for (int i=0; i < 10; i++) {
			id= id +rnd.nextInt(z.size)
		};


		System.out.println(">>>> IMSI _ID = "  +id);
		return id
	}

	@Keyword
	public String getRandomID() {


		String id="1"
		def z = [1, 2, 3, 4, 5, 6, 7, 8, 9]
		Random rnd = new Random()

		for (int i=0; i < 6; i++) {
			id= id +rnd.nextInt(z.size)
		};


		System.out.println(">>>> RANDOM _ID = "  +id);
		return id
	}


	@Keyword
	public void  writeExcelCustID (String custID) throws IOException{
		FileInputStream fis = new FileInputStream("C:\\Users\\7713\\Katalon Studio\\Turkuaz_Test_Project\\Data Files\\deneme.xlsx");
		XSSFWorkbook workbook = new XSSFWorkbook(fis);
		XSSFSheet sheet = workbook.getSheet("Sheet1");
		int rowCount = sheet.getLastRowNum()-sheet.getFirstRowNum();
		Row row = sheet.createRow(1);
		Cell cell = row.createCell(0);
		cell.setCellType(cell.CELL_TYPE_STRING);
		rowCount--;
		cell.setCellValue(custID);
		FileOutputStream fos = new FileOutputStream("C:\\Users\\7713\\Katalon Studio\\Turkuaz_Test_Project\\Data Files\\deneme.xlsx");
		workbook.write(fos);
		fos.close();
	}
}
